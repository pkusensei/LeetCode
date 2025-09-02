using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int[] NextGreaterElements(int[] nums)
    {
        int n = nums.Length;
        int[] vals = [.. nums, .. nums];
        int[] res = new int[n];
        Array.Fill(res, -1);
        Stack<int> st = [];
        for (int i = 0; i < vals.Length; i++)
        {
            while (st.TryPeek(out var top) && vals[top] < vals[i])
            {
                st.Pop();
                if (top < n) { res[top] = vals[i]; }
            }
            st.Push(i);
        }
        return res;
    }
}