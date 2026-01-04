namespace Synthoza.Sylan;

public abstract class Instrument
{
    public double TotalElapsedTime { get; private set; }
    public double DeltaTime { get; private set; }

    protected abstract double Frequency(double pitch, double noteElapsedTime);
    protected abstract double Wave(double timer);

    public double Step(double deltaTime, IEnumerable<Note> notes)
    {
        DeltaTime = deltaTime;

        double totalWave = 0;

        foreach (var note in notes)
        {
            if (note.InTime(TotalElapsedTime))
            {
                var frequency = Frequency(note.Pitch, TotalElapsedTime - note.Start);
                var timer = note.Step(frequency, deltaTime);
                totalWave += Wave(timer);
            }
        }
        
        TotalElapsedTime += deltaTime;
        return totalWave;
    }
}

public class Note(double pitch, double start, double length)
{
    public double Start { get; set; } = start;
    public double Length { get; set; } = length;
    public double Pitch { get; set; } = pitch;
    public double Timer { get; private set; }

    public double End
    {
        get => Start + Length;
        set => Length = value - Start;
    }

    public bool InTime(double time) => Start <= time && time <= End;

    public double Step(double frequency, double deltaTime)
    {
        return Timer = (Timer + frequency * deltaTime) % 1.0;
    }
}