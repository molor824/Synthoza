using System.Collections.Concurrent;
using System.Diagnostics.CodeAnalysis;
using Raylib_cs;
using Synthoza.Sylan;

namespace Synthoza;

public class InstrumentTest : ILoopHandle, IDisposable
{
    private static readonly Note[] Notes = [new(2), new(3)];
    private const double Delta = 1.0 / 44100.0;
    private const int MaxSamples = 2048;
    private const int Timeout = 1;

    private Compiler _compiler = new();
    private Instrument _instance;
    private AudioStream _stream;
    private short[] _buffer = new short[MaxSamples];
    private volatile BlockingCollection<short> _queue = new(new ConcurrentQueue<short>(), MaxSamples);
    private Thread _instrumentThread;
    private bool _disposed;

    public InstrumentTest()
    {
        var instrumentType = _compiler.DefineInstrumentClass("TestInstrument");
        _instance = (Instrument)Activator.CreateInstance(instrumentType, [0.08, 0.25])!;
        _instance = new SubSaw();

        foreach (var note in Notes)
            _instance.AddNote(note);

        Raylib.SetAudioStreamBufferSizeDefault(MaxSamples);
        _stream = Raylib.LoadAudioStream(44100, 16, 1);
        Raylib.PlayAudioStream(_stream);

        _instrumentThread = new Thread(InstrumentThread);
        _instrumentThread.Start();
    }

    void InstrumentThread()
    {
        while (true)
        {
            double wave = _instance.Step(Delta);
            try
            {
                _queue.Add((short)(Math.Clamp(wave * 0.1, -1, 1) * short.MaxValue));
            }
            catch
            {
                break;
            }
        }
    }

    public void Update(float deltaTime)
    {
        if (Raylib.IsAudioStreamProcessed(_stream))
        {
            for (var i = 0; i < _buffer.Length; i++)
            {
                _buffer[i] = _queue.Take();
            }

            unsafe
            {
                fixed (short* ptr = _buffer)
                {
                    Raylib.UpdateAudioStream(_stream, ptr, MaxSamples);
                }
            }
        }
    }

    public void Dispose()
    {
        if (_disposed) return;

        _disposed = true;
        _queue.CompleteAdding();
        _queue.Dispose();
        _instrumentThread.Join();
    }
}

internal class SubSaw : Instrument
{
    public double Height { get; set; } = 0.5;
    public double SubSawFreq { get; set; } = 3.0;

    protected override double Frequency(double pitch, double noteElapsedTime) => Builtin.PitchToFreq(pitch);

    protected override double Wave(double timer)
    {
        var subSaw = timer * SubSawFreq % 1.0 * Height;
        return timer - 0.5 + subSaw;
    }
}