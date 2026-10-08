#!/usr/bin/env python3
"""Publish RealSense D435i raw Z16 depth frames over ZeroMQ."""

import argparse
import signal

import numpy as np
import pyrealsense2 as rs
import zmq


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--bind", default="tcp://0.0.0.0:5555")
    parser.add_argument("--width", type=int, default=848)
    parser.add_argument("--height", type=int, default=480)
    parser.add_argument("--fps", type=int, default=10)
    args = parser.parse_args()

    stopping = False

    def stop(_signum, _frame):
        nonlocal stopping
        stopping = True

    signal.signal(signal.SIGINT, stop)
    signal.signal(signal.SIGTERM, stop)

    context = zmq.Context()
    socket = context.socket(zmq.PUB)
    socket.setsockopt(zmq.LINGER, 0)
    socket.bind(args.bind)

    pipeline = rs.pipeline()
    config = rs.config()
    config.enable_stream(rs.stream.depth, args.width, args.height, rs.format.z16, args.fps)
    pipeline.start(config)
    print(f"depth stream: {args.bind}, {args.width}x{args.height}@{args.fps} Z16", flush=True)

    try:
        while not stopping:
            frames = pipeline.wait_for_frames()
            depth = frames.get_depth_frame()
            if depth is None:
                continue
            frame = np.asanyarray(depth.get_data(), dtype=np.uint16)
            socket.send(frame.tobytes(), copy=False)
    finally:
        pipeline.stop()
        socket.close(0)
        context.term()


if __name__ == "__main__":
    main()
