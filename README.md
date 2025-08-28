# gdpathfinder, or kdnav, or pointnav

A Rust gdextension for A* on a point field with implicit edges.

* gdextension: an extension library for Godot 4.x
* A*: the A* pathfinding algorithm
* point field: a set of points in 3D space
* implicit edges: edges between points are not stored, but calculated on-the-fly

## API

### Scene preparation

Add a KDNav node to your scene.

### Build the kd-tree

In a script, call `kdnav.build(points: Array[Vector3])`.

### Find points

`kdnav.nearest(point: Vector3) -> int`

Returns the index of the nearest point in the point field, or -1 if no such point exists. The index is the same as the index in the Array[Vector3] passed to `build`.

### Find path

`kdnav.path(start_idx: int, end_idx: int, maxdist: float) -> KDNavJob`

Returns a KDNavJob which will find a path from the point at `start_idx` to the point at `end_idx`, using edges between points that are at most `maxdist` apart.

### Wait for the job

The `job.path_found` signal is emitted when the job is complete.

When the job has completed, `job.path` contains an Array[int] of point indices.

## Concurrency

Only one job runs at a time. If you call `kdnav.path` while a job is running, the new job will be queued and run when the current job completes.

There's no way to cancel a job once it's started, yet.
