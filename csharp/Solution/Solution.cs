using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public double MaxAverageRatio(int[][] classes, int extraStudents)
    {
        PriorityQueue<Bunch, double> pq = new(Comparer<double>.Create((a, b) => b.CompareTo(a)));
        foreach (var item in classes)
        {
            Bunch b = new(item[0], item[1]);
            pq.Enqueue(b, b.Increment);
        }
        for (int _ = 0; _ < extraStudents; _++)
        {
            var b = pq.Dequeue();
            b.Pass += 1;
            b.Total += 1;
            pq.Enqueue(b, b.Increment);
        }
        double sum = 0.0;
        while (pq.TryDequeue(out var b, out _))
        {
            sum += b.Ratio;
        }
        return sum / classes.Length;
    }
}

record struct Bunch(int Pass, int Total)
{
    public readonly double Increment => (1.0 + Pass) / (1.0 + Total) - Ratio;
    public readonly double Ratio => (double)Pass / Total;
}