using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public long KMirror(int k, int n)
    {
        long res = 0;
        List<int> s = [0];
        for (int i = 0; i < n; i++)
        {
            while (true)
            {
                s = Build(s, k);
                long num = s.Aggregate(0L, (acc, v) => acc * k + v);
                if (Check(num.ToString()))
                {
                    res += num;
                    break;
                }
            }
        }
        return res;

        static List<int> Build(List<int> s, int k)
        {
            int n = s.Count;
            int mid = n / 2;
            for (int i1 = mid; i1 < n; i1++)
            {
                if (s[i1] + 1 < k)
                {
                    s[i1] += 1;
                    s[n - 1 - i1] = s[i1];
                    for (int i2 = mid; i2 < i1; i2++)
                    {
                        s[i2] = 0;
                        s[n - 1 - i2] = 0;
                    }
                    return s;
                }
            }
            List<int> res = [1];
            res.AddRange(Enumerable.Range(0, n - 1).Select(_ => 0));
            res.Add(1);
            return res;
        }

        static bool Check(string s) => s.SequenceEqual(s.Reverse());
    }
}
