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
        
        self.socket.send_string(json.dumps("Register"))
        
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
            
            if data.get("FormData") is not None and self._pending_request.get("FormData") is not None:
                future = self._pending_request.pop("FormData")
                if not future.done():
                    future.set_result(json.loads(data["FormData"]["data"]))
            elif data.get("NetGraph") is not None and self._pending_request.get("NetGraph") is not None:
                future = self._pending_request.pop("NetGraph")
                if not future.done():
                    future.set_result(data["NetGraph"]["graph"])
            elif data.get("Terminate") is not None:
                await self.socket.send_string(json.dumps("Exit"))
                self.exit()
        
    def toast(self, alert_type: ToastType, text: str):
        """
        Show a toast to the user.
        """
        return self.socket.send_string(json.dumps({
            "Toast": {
                "alert_type": alert_type,
                "text": text
            }
        }))
        
    def execute_raw_query(self, raw_query: str):
        return self.socket.send_string(json.dumps({
            "ExecuteRawQuery": {
                "query": raw_query
            }
        }))
        
    async def form(self, title: str, config: list):
        for i in range(len(config)):
            for j in range(len(config[i])):
                if config[i][j].get("options") is not None:
                    config[i][j]["options"] = [str(x) for x in config[i][j]["options"]]
                if config[i][j].get("min") is not None:
                    config[i][j]["min"] = str(config[i][j]["min"])
                if config[i][j].get("max") is not None:
                    config[i][j]["max"] = str(config[i][j]["max"])
                if config[i][j].get("step") is not None:
                    config[i][j]["step"] = str(config[i][j]["step"])
                if config[i][j].get("default") is not None:
                    config[i][j]["default"] = str(config[i][j]["default"])

        future = asyncio.get_event_loop().create_future()
        self._pending_request["FormData"] = future
        
        await self.socket.send_string(json.dumps({
            "ShowForm": {
                "title": title,
                "config": config
            }
        }))
        
        try:
            response = await future
            return response
        except asyncio.TimeoutError:
            self._pending_request.pop("FormData")
            return None
            
    async def get_net_graph(self):
        """
        Get the graph representing the network. The graph is a petgraph structure serialized to JSON.
        """
        future = asyncio.get_event_loop().create_future()
        self._pending_request["NetGraph"] = future
        
        await self.socket.send_string(json.dumps("GetNetGraph"))
        
        try:
            response = await future
            return response
        except asyncio.TimeoutError:
            self._pending_request.pop("NetGraph")
            return None
            
    def add_net_node(self, node: dict):
        """
        Add a node to the network graph.
        """
        return self.socket.send_string(json.dumps({
            "AddNetNode": {
                "node": node
            }
        }))
        
    def add_net_edge(self, src: int, dst: int):
        """
        Add edge between to nodes in the network graph.
        """
        return self.socket.send_string(json.dumps({
            "AddNetEdge": {
                "src": src,
                "dst": dst
            }
        }))
        
    def remove_net_node(self, node: int):
        """
        Remove node from the network graph.
        """
        return self.socket.send_string(json.dumps({
            "RemoveNetNode": {
                "node": node
            }
        }))
        
    def remove_net_edge(self, edge: int):
        """
        Remove edge between two nodes from the network graph.
        """
        return self.socket.send_string(json.dumps({
            "RemoveNetEdge": {
                "edge": edge
            }
        }))
        
    def update_net_node(self, index: int, node: dict):
        """
        Update a specific node of the network graph.
        """
        return self.socket.send_string(json.dumps({
            "UpdateNetNode": {
                "index": index,
                "node": node
            }
        }))
        
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