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

        if (!_changeRequested) return;
        _changeRequested = false;

        var offset = _pianoRoll.Offset;
        var noteSize = _pianoRoll.NoteSize;
        var start = -offset.PosMod(noteSize);
        var index = (Vector2I)(offset / noteSize);
        var count = (Vector2I)(Size / noteSize).Ceil() + Vector2I.One;

        _verticalContainer.Position = _verticalContainer.Position with
        {
            X = start.X
        };
        _horizontalContainer.Position = _horizontalContainer.Position with
        {
            Y = start.Y
        };

        var verticalChildren = _verticalContainer.GetChildren();
        var horizontalChildren = _horizontalContainer.GetChildren();
            
        for (var i = 0; i < count.X; i++)
        {
            var grid = (VerticalGrid)(i < verticalChildren.Count ? verticalChildren[i] : _verticalGridScene.Instantiate());
            if (i >= verticalChildren.Count) _verticalContainer.AddChild(grid);

            grid.CustomMinimumSize = grid.CustomMinimumSize with
            {
                X = noteSize.X
            };

            var bar = index.X + i;

            grid.Modulate = grid.Modulate with
            {
                A = bar % _pianoRoll.Bars != 0 ? _opacity : _highlightOpacity
            };
            
            grid.Measure = bar % _pianoRoll.Bars == 0 ? bar / _pianoRoll.Bars : null;
        }
        for (var i = 0; i < count.Y; i++)
        {
            var grid = (Control)(i < horizontalChildren.Count ? horizontalChildren[i] : _horizontalGridScene.Instantiate());
            if (i >= horizontalChildren.Count) _horizontalContainer.AddChild(grid);

            grid.CustomMinimumSize = grid.CustomMinimumSize with
            {
                Y = noteSize.Y
            };

            grid.Modulate = grid.Modulate with
            {
                A = _opacity
            };
        }

        for (var i = count.X; i < verticalChildren.Count; i++) verticalChildren[i].QueueFree();
        for (var i = count.Y; i < horizontalChildren.Count; i++) horizontalChildren[i].QueueFree();
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
