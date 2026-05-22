using Godot;

public partial class NoteGrid : Control
{
    private bool _mouseIn, _dragging, _changeRequested = true;

    [Export] private float _opacity = 0.5f;
    [Export] private float _highlightOpacity = 1;
    [Export] private PianoRoll _pianoRoll;
    [Export] private HBoxContainer _verticalContainer;
    [Export] private VBoxContainer _horizontalContainer;
    [Export] private PackedScene _verticalGridScene, _horizontalGridScene;

    private void RequestChange() => _changeRequested = true;

    public override void _Ready()
    {
        base._Ready();

        MouseEntered += () => _mouseIn = true;
        MouseExited += () => _mouseIn = false;

        Resized += RequestChange;
        _pianoRoll.OffsetChanged += RequestChange;
        _pianoRoll.NoteSizeChanged += RequestChange;
        _pianoRoll.TimeSignatureChanged += RequestChange;
    }

    public override void _Process(double delta)
    {
        base._Process(delta);

        if (_changeRequested)
        {
            _changeRequested = false;

            var offset = _pianoRoll.Offset;
            var noteSize = _pianoRoll.NoteSize;
            var start = -offset.PosMod(noteSize);
            var index = (Vector2I)(offset / noteSize);
            var count = (Vector2I)(Size / noteSize).Ceil() + Vector2I.One;

            var position = _verticalContainer.Position;
            position.X = start.X;
            _verticalContainer.Position = position;

            position = _horizontalContainer.Position;
            position.Y = start.Y;
            _horizontalContainer.Position = position;

            var verticalChildren = _verticalContainer.GetChildren();

            for (var i = 0; i < count.X; i++)
            {
                var grid = (Control)(i < verticalChildren.Count ? verticalChildren[i] : _verticalGridScene.Instantiate());
                if (i >= verticalChildren.Count) _verticalContainer.AddChild(grid);

                var minSize = grid.CustomMinimumSize;
                minSize.X = noteSize.X;
                grid.CustomMinimumSize = minSize;

                var modulate = grid.Modulate;
                modulate.A = (index.X + i) % _pianoRoll.Bars != 0 ? _opacity : _highlightOpacity;
                grid.Modulate = modulate;
            }

            var horizontalChildren = _horizontalContainer.GetChildren();

            for (var i = 0; i < count.Y; i++)
            {
                var grid = (Control)(i < horizontalChildren.Count ? horizontalChildren[i] : _horizontalGridScene.Instantiate());
                if (i >= horizontalChildren.Count) _horizontalContainer.AddChild(grid);

                var minSize = grid.CustomMinimumSize;
                minSize.Y = noteSize.Y;
                grid.CustomMinimumSize = minSize;

                var modulate = grid.Modulate;
                modulate.A = _opacity;
                grid.Modulate = modulate;
            }

            for (var i = count.X; i < verticalChildren.Count; i++) verticalChildren[i].QueueFree();
            for (var i = count.Y; i < horizontalChildren.Count; i++) horizontalChildren[i].QueueFree();
        }
    }

    public override void _GuiInput(InputEvent @event)
    {
        base._GuiInput(@event);

        if (@event.IsActionPressed(InputActions.EditorDrag) && _mouseIn) _dragging = true;
        else if (@event.IsActionReleased(InputActions.EditorDrag)) _dragging = false;

        if (_dragging && @event is InputEventMouseMotion mouseMotion)
            _pianoRoll.Offset -= new Vector2(mouseMotion.Relative.X, 0);
    }
}
