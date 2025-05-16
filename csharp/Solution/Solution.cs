using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public long CountGoodIntegers(int n, int k)
    {
        int half = (1 + n) / 2;
        int min = (int)Math.Pow(10, half - 1);
        int max = (int)Math.Pow(10, half);
        HashSet<string> seen = [];
        Span<int> freq = stackalloc int[10];
        long res = 0;
        for (int val = min; val < max; val++)
        {
            freq.Clear();
            string left = val.ToString();
            StringBuilder sb = new(left);
            var ca = left.ToCharArray();
            Array.Reverse(ca);
            sb.Append(ca[(n & 1)..]);
            string s = sb.ToString();
            if (!long.TryParse(s, out var parsed) || parsed % k != 0)
            {
                continue;
            }
            ca = s.ToCharArray();
            Array.Sort(ca);
            s = new(ca);
            if (!seen.Add(s)) { continue; }
            foreach (var item in s)
            {
                freq[item - '0'] += 1;
            }
            int perm = (n - freq[0]) * Fact(n - 1);
            foreach (var item in freq)
            {
                if (item > 1) { perm /= Fact(item); }
            }
            res += perm;
        }
        return res;

        static int Fact(int n) => n < 2 ? 1 : n * Fact(n - 1);
    }
}
