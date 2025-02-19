using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public string GetHappyString(int n, int k)
    {
        StringBuilder sb = new(n);
        string res = string.Empty;
        Backtrack(n, ref k);
        return res;

        bool Backtrack(int n, ref int k)
        {
            if (n == 0)
            {
                k -= 1;
                if (k == 0)
                {
                    res = sb.ToString();
                    return true;
                }
                return false;
            }
            foreach (var item in "abc")
            {
                if (sb.Length > 0 && sb[^1] == item) { continue; }
                sb.Append(item);
                if (Backtrack(n - 1, ref k)) { return true; }
                sb.Remove(sb.Length - 1, 1);
            }
            return false;
        }
    }

    public string WithStack(int n, int k)
    {
        Stack<string> st = [];
        st.Push(string.Empty);
        int sorted_count = 0;
        while (st.TryPop(out var curr))
        {
            if (curr.Length == n)
            {
                sorted_count += 1;
                if (sorted_count == k) { return curr; }
                continue;
            }
            foreach (var item in "cba")
            {
                if (curr.LastOrDefault() != item) { st.Push(curr + item); }
            }
        }
        return string.Empty;
    }

    public string WithMath(int n, int k)
    {
        var total = 3 * (1 << (n - 1));
        if (k > total) { return string.Empty; }

        StringBuilder sb = new(n);
        for (int i = 0; i < n; i++) { sb.Append('a'); }
        Dictionary<char, char> next_smallest = new() { { 'a', 'b' }, { 'b', 'a' }, { 'c', 'a' } };
        Dictionary<char, char> next_greatest = new() { { 'a', 'c' }, { 'b', 'c' }, { 'c', 'b' } };
        var start_a = 1;
        var start_b = start_a + (1 << (n - 1));
        var start_c = start_b + (1 << (n - 1));
        if (k < start_b)
        {
            sb[0] = 'a';
            k -= start_a;
        }
        else if (k < start_c)
        {
            sb[0] = 'b';
            k -= start_b;
        }
        else
        {
            sb[0] = 'c';
            k -= start_c;
        }
        for (int idx = 1; idx < n; idx++)
        {
            var mid = 1 << (n - idx - 1);
            if (k < mid) { sb[idx] = next_smallest[sb[idx - 1]]; }
            else { sb[idx] = next_greatest[sb[idx - 1]]; }
            k -= mid;
        }
        return sb.ToString();
    }
}


