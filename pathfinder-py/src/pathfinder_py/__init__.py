import argparse
from enum import Enum
import json
import sys

import jsonschema
import zmq


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
                "language": { "type": "string" }
            }
        }
        
    class QueryBuilder:
        pass
    
    def __init__(self) -> None:
        with open("./config.json", "r") as f:
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
        
        self.context = zmq.Context()
        
        self.socket = self.context.socket(zmq.DEALER)
        self.socket.setsockopt_string(zmq.IDENTITY, self.config["name"])
        
        if args.port < 1 or args.port > 65535:
            raise Exception("Port must be a value between 1 and 65535.")
            
        self.report = args.report
            
        self.socket.connect(f"tcp://localhost:{args.port}") 
        
    def toast(self, alert_type: ToastType, text: str):
        self.socket.send_json({
            "Toast": {
                "alert_type": alert_type,
                "text": text
            }
        })
        
    def execute_raw_query(self, raw_query: str):
        self.socket.send_json({
            "ExecuteRawQuery": {
                "query": raw_query
            }
        })
        
    def show_form(self, config: dict):
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
        self.socket.send_json({
            "FormReq": {
                "data": {
                    "name": self.config["name"],
                    "config": config
                }
            }
        })
        
        return self.socket.recv_json()
        
    def exit(self):
        self.socket.close()
        self.context.term()