using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MinimumPairRemoval(int[] nums)
    {
        int n = nums.Length;
        // Removed `nodes` array 
        // Rely on GC tracing to keep nodes valid
        bool[] merged = new bool[n]; // functions as set
        PriorityQueue<Pair, Pair> pq = new();
        int bad_count = 0;
        Node prev = null;
        for (int i = 0; i < n; i++)
        {
            Node curr = new(i, nums[i]);
            if (i > 0)
            {
                prev.Next = curr;
                curr.Prev = prev;
                Pair p = new(prev, curr, nums[i - 1] + (long)nums[i]);
                pq.Enqueue(p, p);
                if (nums[i - 1] > nums[i]) { bad_count += 1; }
            }
            prev = curr;
        }
        int res = 0;
        while (bad_count > 0 && pq.TryDequeue(out var pair, out _))
        {
            (Node node1, Node node2, long val) = pair;
            if (merged[node1.Idx] || merged[node2.Idx]
                || node1.Val + node2.Val != val)
            {
                continue;
            }
            res += 1;
            if (node1.Val > node2.Val) { bad_count -= 1; }
            prev = node1.Prev;
            Node next = node2.Next;
            node1.Next = next;
            if (next is not null) { next.Prev = node1; }
            if (prev is not null)
            {
                // Before update: [prev]>[i1]
                // After update: [prev]<val
                if (node1.Val < prev.Val && prev.Val <= val)
                {
                    bad_count -= 1;
                }
                // Before [prev]<=[i1]
                // After [prev]>val
                else if (val < prev.Val && prev.Val <= node1.Val)
                {
                    bad_count += 1;
                }
                Pair p = new(prev, node1, prev.Val + val);
                pq.Enqueue(p, p);
            }
            if (next is not null)
            {
                // Before [i2]>[next]
                // After val<=[next]
                if (next.Val < node2.Val && val <= next.Val)
                {
                    bad_count -= 1;
                }
                // Before [i2]<=[next]
                // After val>[next]
                else if (node2.Val <= next.Val && next.Val < val)
                {
                    bad_count += 1;
                }
                Pair p = new(node1, next, next.Val + val);
                pq.Enqueue(p, p);
            }
            node1.Val = val;
            merged[node2.Idx] = true;
        }
        return res;
    }
}

internal sealed class Node(int i, int val)
{
    public int Idx { get; } = i;
    public long Val { get; set; } = val;
    public Node Prev { get; set; }
    public Node Next { get; set; }
}

internal struct Pair(Node n1, Node n2, long val) : IComparable<Pair>
{
    public Node Node1 { get; set; } = n1;
    public Node Node2 { get; set; } = n2;
    public long Val { get; set; } = val;

    public readonly int CompareTo(Pair other)
        => Val == other.Val
           ? Node1.Idx.CompareTo(other.Node1.Idx)
           : Val.CompareTo(other.Val);

    public readonly void Deconstruct(out Node n1, out Node n2, out long val)
    {
        n1 = Node1;
        n2 = Node2;
        val = Val;
    }
}