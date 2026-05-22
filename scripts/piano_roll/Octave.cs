using Godot;

public partial class Octave : Control
{
    [Export] private Label _label;
    [Export] private int _index = 0;

    public int Index
    {
        get => _index;
        set
        {
            _index = value;
            _label.Text = value.ToString();
        }
    }

    public override void _Ready()
    {
        base._Ready();

        Index = _index;
    }
}