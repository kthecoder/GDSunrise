extends Node

var webPinkServer : WarpKuzuServer = WarpKuzuServer.new();

func _ready() -> void:
	webPinkServer.init_db("my_database")
	webPinkServer.set_static_folder("my_vue_app")
	webPinkServer.set_port(8080)
	webPinkServer.start_server()
	webPinkServer.start_websocket()

	# Notify clients when data changes
	webPinkServer.notify_clients("New data available!")

