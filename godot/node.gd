extends Node

func _ready() -> void:
  var x := AStarKD3D.new()
  var r := x.build([
    Vector3.UP,
    Vector3.DOWN,
    Vector3.LEFT,
    Vector3.RIGHT,
    Vector3.FORWARD,
    Vector3.BACK,
  ], 5)
  print(r)
