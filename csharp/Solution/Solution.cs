using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class TopVotedCandidate
{
    int[] Times { get; }
    int[] Leading { get; }
    int Length { get; }

    public TopVotedCandidate(int[] persons, int[] times)
    {
        Times = times;
        Length = times.Length;
        Dictionary<int, int> freq = [];
        List<int> leading = new(Length);
        int max = 0;
        foreach (var (person, time) in persons.Zip(times))
        {
            if (!freq.TryAdd(person, 1)) { freq[person] += 1; }
            max = int.Max(max, freq[person]);
            if (max == freq[person]) { leading.Add(person); }
            else { leading.Add(leading.Last()); }
        }
        Leading = [.. leading];
    }

    public int Q(int t)
    {
        int i = Array.BinarySearch(Times, t);
        if (i < 0) { i = (~i) - 1; }
        return Leading[i];
    }
}