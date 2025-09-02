using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public Solution(int[][] rects)
    {
        N = rects.Length;
        Rects = new(N);
        Prefix = new(N);
        Rng = new();
        foreach (var r in rects)
        {
            Rect curr = new(r[0], r[1], r[2], r[3]);
            Rects.Add(curr);
            Prefix.Add(curr.Dots + Prefix.LastOrDefault());
        }
    }

    int N { get; }
    List<Rect> Rects { get; }
    List<long> Prefix { get; }
    Random Rng { get; }

    public int[] Pick()
    {
        long all_dots = Prefix[^1];
        long count = Rng.NextInt64(1, 1 + all_dots);
        int i = Prefix.BinarySearch(count);
        if (i < 0) { i = ~i; }
        return Rects[i].Point(Rng);
    }
}

readonly record struct Rect(int X1, int Y1, int X2, int Y2)
{
    public readonly long Dots => (X2 - X1 + 1L) * (Y2 - Y1 + 1L);
    public readonly int[] Point(Random rng) => [rng.Next(X1, 1 + X2), rng.Next(Y1, 1 + Y2)];
}