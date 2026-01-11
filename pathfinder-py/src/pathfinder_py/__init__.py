import __main__
import argparse
import asyncio
from enum import Enum
import json
import os
import sys

import jsonschema
import zmq
import zmq.asyncio


class Plugin:
    class ToastType(int, Enum):
        NONE = 0
        SUCCESS = 1
        INFO = 2
        WARNING = 3
        DANGER = 4
    
    class Config:
        schema = {
            "type": "object",
            "properties": {
                "name": { "type": "string" },
                "author": { "type": "string" },
                "license": { "type": "string" },
                "repository": { "type": "string" },
                "version": { "type": "string" },
                "language": { "type": "string" },
                "permissions": { "type": "string" }
            }
        }
        
    class QueryBuilder:
        pass
    
    def __init__(self) -> None:
        with open(os.path.join(os.path.dirname(os.path.realpath(__main__.__file__)), "../config.json"), "r") as f:
            self.config = json.loads(f.read())
            jsonschema.validate(instance=self.config, schema=self.Config.schema)
        
        parser = argparse.ArgumentParser(prog=self.config["name"])
        parser.add_argument(
            "--port",
            type=int,
            default=5555,
            help="Port of the plugins (0MQ) server."
        )
        parser.add_argument(
            "--report",
            help="UUID of the report currently loaded."
        )
        args = parser.parse_args(sys.argv[1:])
        
        if args.port < 1 or args.port > 65535:
            raise Exception("Port must be a value between 1 and 65535.")
            
        self.report = args.report
            
        self.context = zmq.asyncio.Context()
        self.socket = self.context.socket(zmq.DEALER)
        self.socket.setsockopt_string(zmq.IDENTITY, self.config["name"])
        self.socket.connect(f"tcp://localhost:{args.port}")
        self._pending_request = {}
        
        self._closing = False
        self._listen_task = asyncio.create_task(self._listen())
        self._shutdown_task = None
        
    async def _listen(self):
        while True:
            try:
                raw = await self.socket.recv_string()
            except asyncio.CancelledError:
                break
            except Exception:
                if self._closing:
                    break
                else:
                    try:
                        print("recv_string error in _listen()", file=sys.stderr)
                    except Exception:
                        pass
                    await asyncio.sleep(0)
                    continue
            
            try:
                data = json.loads(raw)
            except Exception:
                continue
            
            if data.get("FormRes") is not None and self._pending_request.get("FormRes") is not None:
                future = self._pending_request.pop("FormRes")
                if not future.done():
                    future.set_result(json.loads(data["FormRes"]["data"]))
            elif data.get("TerminateCmd") is not None:
                self.exit()
                break
        
    def toast(self, alert_type: ToastType, text: str):
        return self.socket.send_string(json.dumps({
            "ToastReq": {
                "alert_type": alert_type,
                "text": text
            }
        }))
        
    def execute_raw_query(self, raw_query: str):
        return self.socket.send_string(json.dumps({
            "ExecuteRawQueryReq": {
                "query": raw_query
            }
        }))
        
    async def form(self, config: dict):
        for f in config.keys():
            if config[f].get("options") is not None:
                config[f]["options"] = [str(i) for i in config[f]["options"]]
            if config[f].get("min") is not None:
                config[f]["min"] = str(config[f]["min"])
            if config[f].get("max") is not None:
                config[f]["max"] = str(config[f]["max"])
            if config[f].get("step") is not None:
                config[f]["step"] = str(config[f]["step"])
            if config[f].get("default") is not None:
                config[f]["default"] = str(config[f]["default"])
        
        future = asyncio.get_event_loop().create_future()
        self._pending_request["FormRes"] = future
        
        await self.socket.send_string(json.dumps({
            "FormReq": {
                "data": {
                    "name": self.config["name"],
                    "config": config
                }
            }
        }))
        
        try:
            response = await future
            return response
        except asyncio.TimeoutError:
            self._pending_request.pop("FormRes")
            return None
        
    def exit(self):
        if self._shutdown_task is None:
            self._shutdown_task = asyncio.create_task(self._shutdown())
        
    async def _shutdown(self):
        if self._closing:
            return
        self._closing = True
        
        for key, fut in list(self._pending_request.items()):
            if not fut.done():
                try:
                    fut.set_exception(RuntimeError("Plugin is shutting down"))
                except Exception:
                    pass
            self._pending_request.pop(key, None)
            
        if self._listen_task is not None:
            self._listen_task.cancel()
            try:
                await self._listen_task
            except asyncio.CancelledError:
                pass
            except Exception:
                pass
            finally:
                self._listen_task = None
                
        try:
            self.socket.close()
        except Exception:
            pass
        try:
            self.context.term()
        except Exception:
            pass