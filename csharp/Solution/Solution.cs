using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class MinStack
{
    public Stack<int> St { get; }
    public Stack<int> MinSt { get; }

    public MinStack()
    {
        St = [];
        MinSt = [];
    }

    public void Push(int val)
    {
        St.Push(val);
        if (MinSt.Count == 0 || MinSt.Peek() >= val)
        {
            MinSt.Push(val);
        }
        else
        {
            MinSt.Push(MinSt.Peek());
        }
    }

    public void Pop()
    {
        St.Pop();
        MinSt.Pop();
    }

    public int Top() => St.Peek();

    public int GetMin() => MinSt.Peek();
}