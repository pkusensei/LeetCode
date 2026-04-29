using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int OddEvenJumps(int[] arr)
    {
        int n = arr.Length;
        List<(int num, int idx)> sorted = new(n);
        for (int i = 0; i < n; i++)
        {
            sorted.Add((arr[i], i));
        }
        sorted.Sort();
        Stack<int> st = [];
        int[] odd_jumps = new int[n];
        Array.Fill(odd_jumps, -1);
        foreach (var (_, idx) in sorted)
        {
            while (st.TryPeek(out var top) && top < idx)
            {
                st.Pop();
                odd_jumps[top] = idx;
            }
            st.Push(idx);
        }
        sorted.Sort((a, b) => a.num == b.num ? a.idx.CompareTo(b.idx) : b.num.CompareTo(a.num));
        st.Clear();
        int[] even_jumps = new int[n];
        Array.Fill(even_jumps, -1);
        foreach (var (_, idx) in sorted)
        {
            while (st.TryPeek(out var top) && top < idx)
            {
                st.Pop();
                even_jumps[top] = idx;
            }
            st.Push(idx);
        }
        int[,] memo = new int[n, 2];
        for (int i = 0; i < n; i++)
        {
            for (int odd = 0; odd < 2; odd++)
            {
                memo[i, odd] = -1;
            }
        }
        int res = 0;
        for (int i = 0; i < n; i++)
        {
            res += Dfs(i, 1);
        }
        return res;

        int Dfs(int idx, int odd)
        {
            if (idx == n - 1) { return 1; }
            if (idx == -1) { return 0; }
            if (memo[idx, odd] > -1) { return memo[idx, odd]; }
            int res = 0;
            if (odd == 1)
            {
                res = Dfs(odd_jumps[idx], odd ^ 1);
            }
            else
            {
                res = Dfs(even_jumps[idx], odd ^ 1);
            }
            memo[idx, odd] = res;
            return res;
        }
    }
}
