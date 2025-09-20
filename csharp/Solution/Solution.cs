using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Router
{
    public Router(int memoryLimit)
    {
        Cap = memoryLimit;
        Set = [];
        Fifo = [];
        DstTimes = [];
    }

    int Cap { get; }
    HashSet<Packet> Set { get; }
    Queue<Packet> Fifo { get; }
    Dictionary<int, List<int>> DstTimes { get; }

    public bool AddPacket(int source, int destination, int timestamp)
    {
        Packet p = new(source, destination, timestamp);
        if (!Set.Add(p)) { return false; }
        if (Fifo.Count == Cap) { ForwardPacket(); }
        Fifo.Enqueue(p);
        if (!DstTimes.TryAdd(destination, [timestamp])) { DstTimes[destination].Add(timestamp); }
        return true;
    }

    public int[] ForwardPacket()
    {
        if (!Fifo.TryDequeue(out var pac)) { return []; }
        Set.Remove(pac);
        DstTimes[pac.Dst].RemoveAt(0);
        return [pac.Src, pac.Dst, pac.Time];
    }

    public int GetCount(int destination, int startTime, int endTime)
    {
        if (!DstTimes.TryGetValue(destination, out var list) || list.Count == 0)
        {
            return 0;
        }
        int start = list.BinarySearch(startTime);
        if (start >= 0)
        {
            while (start >= 0 && list[start] == startTime)
            {
                start -= 1;
            }
            start += 1;
        }
        else
        {
            start = ~start;
        }
        int end = list.BinarySearch(endTime);
        if (end >= 0)
        {
            while (end < list.Count && list[end] == endTime)
            {
                end += 1;
            }
        }
        else
        {
            end = ~end;
        }
        return end - start;
    }
}

readonly record struct Packet(int Src, int Dst, int Time);
