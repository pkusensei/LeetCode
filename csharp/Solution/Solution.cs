using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;


public class MyQueue
{
    readonly Stack<int> in_st;
    readonly Stack<int> out_st;

    public MyQueue()
    {
        in_st = [];
        out_st = [];
    }

    public void Push(int x)
    {
        while (out_st.TryPop(out var num))
        {
            in_st.Push(num);
        }
        in_st.Push(x);
    }

    public int Pop()
    {
        while (in_st.TryPop(out var num))
        {
            out_st.Push(num);
        }
        return out_st.Pop();
    }

    public int Peek()
    {
        while (in_st.TryPop(out var num))
        {
            out_st.Push(num);
        }
        return out_st.Peek();
    }

    public bool Empty() => in_st.Count == 0 && out_st.Count == 0;
}