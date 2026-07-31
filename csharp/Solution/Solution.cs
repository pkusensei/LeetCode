using System.Collections.Frozen;
using System.Linq.Expressions;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class DinnerPlates
{

    public DinnerPlates(int capacity)
    {
        Cap = capacity;
        Stacks = [];
        Free = [];
    }

    int Cap { get; }
    List<Stack<int>> Stacks { get; }
    SortedSet<int> Free { get; }

    public void Push(int val)
    {
        if (Free.Count > 0)
        {
            int i = Free.Min;
            Stacks[i].Push(val);
            if (Stacks[i].Count >= Cap) { Free.Remove(i); }
        }
        else
        {
            Stack<int> st = new();
            st.Push(val);
            Stacks.Add(st);
            if (Cap > 1) { Free.Add(Stacks.Count - 1); }
        }
    }

    public int Pop()
    {
        if (Stacks.Count == 0) { return -1; }
        int res = Stacks[^1].Pop();
        Free.Add(Stacks.Count - 1);
        while (Stacks.Count > 0 && Stacks[^1].Count == 0)
        {
            Free.Remove(Stacks.Count - 1);
            Stacks.RemoveAt(Stacks.Count - 1);
        }
        return res;
    }

    public int PopAtStack(int index)
    {
        if (index >= Stacks.Count || Stacks[index].Count == 0) { return -1; }
        if (index == Stacks.Count - 1) { return Pop(); }
        int res = Stacks[index].Pop();
        Free.Add(index);
        return res;
    }
}
