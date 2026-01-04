using Raylib_cs;
using Synthoza.Sylan;

namespace Synthoza;

public class InstrumentTest : ILoopHandle
{
    private static readonly Note[] Notes = [new(3, 0, 100), new(3 + 7.0 / 12.0, 0, 100)];
    private const double Delta = 1.0 / 44100.0;
    private const int MaxSamples = 2048;

    private Compiler _compiler = new();
    private Type _instrumentType;
    private Instrument _instance;
    private AudioStream _stream;
    private short[] _buffer = new short[MaxSamples];

    public InstrumentTest()
    {
        _instrumentType = _compiler.DefineInstrumentClass("TestInstrument");
        _instance = (Instrument)Activator.CreateInstance(_instrumentType, [0.08, 0.5])!;
        
        Raylib.SetAudioStreamBufferSizeDefault(MaxSamples);
        _stream = Raylib.LoadAudioStream(44100, 16, 1);
        Raylib.PlayAudioStream(_stream);
    }

    public void Update(float deltaTime)
    {
        if (Raylib.IsAudioStreamProcessed(_stream))
        {
            for (var i = 0; i < _buffer.Length; i++)
                _buffer[i] = (short)(Math.Clamp(_instance.Step(Delta, Notes) * 0.1f, -1, 1) * short.MaxValue);
            unsafe
            {
                fixed (short* ptr = _buffer)
                {
                    Raylib.UpdateAudioStream(_stream, ptr, MaxSamples);
                }
            }
        }
    }
}