using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Skiplist
{
    const int MAX_LEVEL = 20;
    Node Head { get; }
    int Level { get; set; }
    Random Rand { get; }

    public Skiplist()
    {
        Head = new(-1, MAX_LEVEL);
        Level = 0;
        Rand = new();
    }

    public bool Search(int target)
    {
        var curr = Head;
        for (int i = Level; i >= 0; i -= 1)
        {
            while (curr.Forward[i] is not null && curr.Forward[i].Val < target)
            {
                curr = curr.Forward[i];
            }
        }
        curr = curr.Forward[0];
        return curr is not null && curr.Val == target;
    }

    public void Add(int num)
    {
        var curr = Head;
        var update = new Node[1 + MAX_LEVEL];

        for (int i = Level; i >= 0; i -= 1)
        {
            while (curr.Forward[i] is not null && curr.Forward[i].Val < num)
            {
                curr = curr.Forward[i];
            }
            update[i] = curr;
        }
        var new_level = RandomLevel();
        if (new_level > Level)
        {
            for (int i = 1 + Level; i <= new_level; i++)
            {
                update[i] = Head;
            }
            Level = new_level;
        }
        Node node = new(num, new_level);
        for (int i = 0; i <= new_level; i++)
        {
            node.Forward[i] = update[i].Forward[i];
            update[i].Forward[i] = node;
        }

        int RandomLevel()
        {
            int res = 0;
            while ((Rand.Next(2) & 1) == 1 && res < MAX_LEVEL)
            {
                res += 1;
            }
            return res;
        }
    }

    public bool Erase(int num)
    {
        var update = new Node[1 + MAX_LEVEL];
        var curr = Head;

        for (int i = Level; i >= 0; i -= 1)
        {
            while (curr.Forward[i] is not null && curr.Forward[i].Val < num)
            {
                curr = curr.Forward[i];
            }
            update[i] = curr;
        }
        curr = curr.Forward[0];
        if (curr is null || curr.Val != num) { return false; }
        for (int i = 0; i <= Level; i++)
        {
            if (update[i].Forward[i] != curr) { break; }
            update[i].Forward[i] = curr.Forward[i];
        }
        while (Level > 0 && Head.Forward[Level] is null)
        {
            Level -= 1;
        }
        return true;
    }
}

public class Node(int val, int level)
{
    public int Val { get; } = val;
    public Node[] Forward { get; } = new Node[1 + level];
}
