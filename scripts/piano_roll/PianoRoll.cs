using System;
using Godot;

public partial class PianoRoll : Control
{
    [Export] private Vector2 _noteSize = new(24 * 4, 24);
    [Export] private int _bars = 4;
    [Export] private int _crotchets = 4;

    private bool _mouseEntered, _dragging;
    private Vector2 _offset;

    [Signal] public delegate void NoteSizeChangedEventHandler();
    [Signal] public delegate void OffsetChangedEventHandler();
    [Signal] public delegate void TimeSignatureChangedEventHandler();

    public Vector2 Offset
    {
        get => _offset;
        set
        {
            value = value.Max(Vector2.Zero);
            if (value == _offset) return;
            _offset = value;
            EmitSignal(SignalName.OffsetChanged);
        }
    }
    public Vector2 NoteSize
    {
        get => _noteSize;
        set
        {
            value = value.Max(Vector2.Zero);
            if (value == _noteSize) return;
            _noteSize = value;
            EmitSignal(SignalName.NoteSizeChanged);
        }
    }
    public int Bars
    {
        get => _bars;
        set
        {
            value = Math.Max(value, 1);
            if (value == _bars) return;
            _bars = value;
            EmitSignal(SignalName.TimeSignatureChanged);
        }
    }
    public int Crotchets
    {
        get => _crotchets;
        set
        {
            value = Math.Max(value, 1);
            if (value == _crotchets) return;
            _crotchets = value;
            EmitSignal(SignalName.TimeSignatureChanged);
        }
    }

    public override void _Ready()
    {
        base._Ready();

        MouseEntered += () => _mouseEntered = true;
        MouseExited += () => _mouseEntered = false;
    }

    public override void _GuiInput(InputEvent @event)
    {
        base._GuiInput(@event);

        if (@event.IsActionPressed(InputActions.EditorDrag) && _mouseEntered) _dragging = true;
        else if (@event.IsActionReleased(InputActions.EditorDrag)) _dragging = false;

        if (_dragging && @event is InputEventMouseMotion mouseMotion)
            Offset -= new Vector2(0, mouseMotion.Relative.Y);
    }
}
