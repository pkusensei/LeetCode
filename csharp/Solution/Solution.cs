using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int[] ExclusiveTime(int n, IList<string> logs)
    {
        int[] res = new int[n];
        Stack<(int id, int time)> st = [];
        foreach (var log in logs)
        {
            var s = log.Split(':');
            int id = int.Parse(s[0]);
            int time = int.Parse(s[2]);
            if (s[1] == "start")
            {
                if (st.TryPeek(out var top))
                {
                    res[top.id] += time - top.time;
                }
                st.Push((id, time));
            }
            else
            {
                var top = st.Pop();
                res[top.id] += 1 + time - top.time;
                if (st.TryPop(out var item))
                {
                    item.time = 1 + time;
                    st.Push(item);
                }
            }
        }
        return res;
    }
}