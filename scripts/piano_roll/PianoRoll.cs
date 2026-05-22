using System;
using Godot;

public partial class PianoRoll : Control
{
    [Export] private Vector2 _noteSize = new(24 * 4, 24);
    [Export] private int _bars = 4;
    [Export] private int _crotchets = 4;

    private bool _mouseEntered, _dragging;
    private Vector2 _offset;

    public event Action NoteSizeChanged;
    public event Action OffsetChanged;
    public event Action TimeSignatureChanged;

    public Vector2 Offset
    {
        get => _offset;
        set
        {
            value = value.Max(Vector2.Zero);
            if (value == _offset) return;
            _offset = value;
            OffsetChanged?.Invoke();
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
            NoteSizeChanged?.Invoke();
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
            TimeSignatureChanged?.Invoke();
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
            TimeSignatureChanged?.Invoke();
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
