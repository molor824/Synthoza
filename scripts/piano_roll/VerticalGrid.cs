using Godot;

public partial class VerticalGrid : Control
{
	[Export] private Label _label;

	private int? _measure;

	public int? Measure
	{
		get => _measure;
		set
		{
			_measure = value;
			_label.Text = _measure.HasValue ? _measure.Value.ToString() : "";
		}
	}

	public override void _Ready()
	{
		base._Ready();
		Measure = _measure;
	}
}
