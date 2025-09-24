using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public bool IsPossible(int[] nums)
    {
        PriorityQueue<Seq, Seq> pq = new();
        foreach (var num in nums)
        {
            while (true)
            {
                if (pq.TryPeek(out Seq seq, out _))
                {
                    if (seq.Num + 1 == num)
                    {
                        pq.Dequeue(); // Continue on this seq
                        seq = new(num, 1 + seq.Len);
                        pq.Enqueue(seq, seq);
                        break;
                    }
                    else if (seq.Num == num)
                    {
                        seq = new(num, 1); // start a new one
                        pq.Enqueue(seq, seq);
                        break;
                    }
                    else
                    {
                        // Found gap; clear pq
                        if (seq.Len < 3) { return false; }
                        else { pq.Dequeue(); }
                    }
                }
                else
                {
                    seq = new(num, 1);
                    pq.Enqueue(seq, seq);
                    break;
                }
            }
        }
        while (pq.TryDequeue(out var seq, out _))
        {
            if (seq.Len < 3) { return false; }
        }
        return true;
    }

    readonly record struct Seq(int Num, int Len) : IComparable<Seq>
    {
        public int CompareTo(Seq other)
        => Num == other.Num ? Len.CompareTo(other.Len) : Num.CompareTo(other.Num);
    }
}