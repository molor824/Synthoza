using Godot;

public partial class Piano : Control
{
	[Export] private VBoxContainer _octaveScroll;
	[Export] private PianoRoll _pianoRoll;
	[Export] private PackedScene _octaveScene;

	private bool _changeRequested = true;

	private void RequestChange() => _changeRequested = true;

	public override void _Ready()
	{
		base._Ready();

		Resized += RequestChange;
		_pianoRoll.OffsetChanged += RequestChange;
		_pianoRoll.NoteSizeChanged += RequestChange;
	}

	public override void _Process(double delta)
	{
		base._Process(delta);

		if (!_changeRequested) return;
		_changeRequested = false;

		var octaveHeight = _pianoRoll.NoteSize.Y * 12;
		var offset = _pianoRoll.Offset.Y;

		_octaveScroll.Position = _octaveScroll.Position with
		{
			Y = -Mathf.PosMod(offset, octaveHeight)
		};

		var startOctave = Mathf.FloorToInt(offset / octaveHeight);
		var endOctave = Mathf.CeilToInt((offset + Size.Y) / octaveHeight);
		var octaves = endOctave - startOctave;
		var children = _octaveScroll.GetChildren();
		for (var i = 0; i < octaves; i++)
		{
			var octave = i < children.Count ? (Octave)children[i] : _octaveScene.Instantiate<Octave>();
			octave.Index = i + startOctave;
			octave.CustomMinimumSize = octave.CustomMinimumSize with
			{
				Y = octaveHeight
			};
			if (i >= children.Count) _octaveScroll.AddChild(octave);
		}

		for (var i = octaves; i < children.Count; i++) children[i].QueueFree();
	}
}
