namespace Synthoza.Sylan;

public abstract class Instrument
{
    public double TotalElapsedTime { get; private set; }
    public double DeltaTime { get; private set; }

    protected abstract double Frequency(double pitch, double noteElapsedTime);
    protected abstract double Wave(double timer);

    private HashSet<Note> _notes = [];

    public bool AddNote(Note note) => _notes.Add(note);
    public bool RemoveNote(Note note) => _notes.Remove(note);

    public double Step(double deltaTime)
    {
        DeltaTime = deltaTime;

        double totalWave = 0;

        foreach (var note in _notes)
        {
            var frequency = Frequency(note.Pitch, note.ElapsedTime);
            note.Step(frequency, deltaTime);
            totalWave += Wave(note.Timer);
        }
        
        TotalElapsedTime += deltaTime;
        return totalWave;
    }
}

public class Note(double pitch)
{
    public double Pitch { get; set; } = pitch;
    public double Timer { get; private set; }
    public double ElapsedTime { get; private set; }

    public void Step(double frequency, double deltaTime)
    {
        ElapsedTime += deltaTime;
        Timer = (Timer + frequency * deltaTime) % 1.0;
    }
}