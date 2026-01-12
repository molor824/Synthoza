using System.Diagnostics;
using System.Diagnostics.CodeAnalysis;

namespace Synthoza;

public class PianoRollStorage(EntityStorage entityStorage, HierarchyStorage hierarchyStorage)
    : IStorage<PianoRollState>, ILoopHandle
{
    private Dictionary<Entity, PianoRollState> _pianoRolls = [];

    public void AddEntity(Entity entity, PianoRollState data)
    {
        _pianoRolls[entity] = data;
    }

    public void DeleteEntity(Entity entity)
    {
        _pianoRolls.Remove(entity);
    }

    public void Update(float deltaTime)
    {
        foreach (var (entity, pianoRoll) in _pianoRolls)
        {
            if (pianoRoll.Modified)
            {
                while (pianoRoll.TryPollCommand(out var command))
                {
                    switch (command)
                    {
                        case PianoRollState.NoteAdded noteAddedCommand:
                            Console.WriteLine("Note added!");
                            break;
                        case PianoRollState.NoteRemoved noteRemovedCommand:
                            Console.WriteLine("Note removed!");
                            break;
                        default:
                            throw new UnreachableException();
                    }
                }
            }
            if (pianoRoll.NotesModified)
            {
                
            }
        }
    }
}

public class PianoRollState
{
    public record NoteAdded(NoteState note);

    public record NoteRemoved(NoteState note);

    public bool Modified => _commands.Count != 0;
    public bool NotesModified => _notes.Any(n => n.Modified);

    private HashSet<NoteState> _notes = [];
    private Queue<object> _commands = [];

    public void AddNoteState(NoteState noteState)
    {
        _commands.Enqueue(new NoteAdded(noteState));
        _notes.Add(noteState);
    }

    public void RemoveNoteState(NoteState noteState)
    {
        _commands.Enqueue(new NoteRemoved(noteState));
        _notes.Remove(noteState);
    }

    public IReadOnlySet<NoteState> GetNoteStates() => _notes;

    public bool TryPollCommand([MaybeNullWhen(false)] out object component) => _commands.TryDequeue(out component);
}

public class NoteState(TimeSpan start, TimeSpan length, int octave, int index)
{
    private TimeSpan _start = start;
    private TimeSpan _length = length;
    private int _octave = octave;
    private int _index = index;

    public bool Modified { get; private set; } = true;

    public TimeSpan Start
    {
        get => _start;
        set => Set(ref _start, value);
    }

    public TimeSpan Length
    {
        get => _length;
        set => Set(ref _length, value);
    }

    public int Octave
    {
        get => _octave;
        set => Set(ref _octave, value);
    }

    public int Index
    {
        get => _index;
        set => Set(ref _index, value);
    }

    void Set<T>(ref T field, T value) where T : IEquatable<T>
    {
        if (field.Equals(value)) return;
        field = value;
        Modified = true;
    }

    public void ClearModified() => Modified = false;
}