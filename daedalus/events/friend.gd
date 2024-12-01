extends Decision

func _ready():
	options = ['Hello', 'World', 'Wow!']

func get_flavour_text() -> String:
	return "%s is so skibidi. %s ohio rizzler, yeah %s" % [player.title, enemy_player.title, relevant_nebula.title]

func select_option(idx: int):
	if idx == 0:
		# Do hello things
		pass
	else:
		# Do other things
		pass
