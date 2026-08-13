using System.Collections.Frozen;
using System.Linq.Expressions;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Skiplist
{
    public Skiplist()
    {
        Head = new(-1);
        Rng = new();
    }

    public Node Head { get; private set; }
    public Random Rng { get; }

    public bool Search(int target)
    {
        Node curr = Head;
        while (curr is not null)
        {
            while (curr.Next is not null && curr.Next.Val < target)
            {
                curr = curr.Next;
            }
            if (curr.Next is not null && curr.Next.Val == target)
            {
                return true;
            }
            curr = curr.Down;
        }
        return false;
    }

    public void Add(int num)
    {
        Stack<Node> st = [];
        Node curr = Head;
        while (curr is not null)
        {
            while (curr.Next is not null && curr.Next.Val < num)
            {
                curr = curr.Next;
            }
            st.Push(curr);
            curr = curr.Down;
        }
        bool insert = true;
        Node down = null;
        while (insert && st.TryPop(out curr))
        {
            curr.Next = new(num, curr.Next, down);
            down = curr.Next;
            insert = Rng.NextDouble() < 0.5;
        }
        if (insert) { Head = new(-1, null, Head); }
    }

    public bool Erase(int num)
    {
        Node curr = Head;
        bool found = false;
        while (curr is not null)
        {
            while (curr.Next is not null && curr.Next.Val < num)
            {
                curr = curr.Next;
            }
            if (curr.Next is not null && curr.Next.Val == num)
            {
                curr.Next = curr.Next.Next;
                found = true;
            }
            curr = curr.Down;
        }
        return found;
    }
}

public sealed class Node
{
    public Node(int val, Node next = null, Node down = null)
    {
        Val = val;
        Next = next;
        Down = down;
    }

    public int Val { get; }
    public Node Next { get; set; }
    public Node Down { get; set; }
}