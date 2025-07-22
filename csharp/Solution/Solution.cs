using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public string ShortestPalindrome(string s)
    {
        int len = KMP(s);
        List<char> res = [.. s.Reverse().Take(s.Length - len)];
        res.AddRange(s);
        return string.Concat(res);

        static int KMP(string s)
        {
            List<char> arr = [.. s];
            arr.Add('#');
            arr.AddRange(s.Reverse());
            int n = arr.Count;
            int[] lps = new int[n];
            int len = 0;
            for (int i = 1; i < n; i++)
            {
                while (len > 0 && arr[len] != arr[i])
                {
                    len = lps[len - 1];
                }
                if (arr[len] == arr[i]) { len += 1; }
                lps[i] = len;
            }
            return lps[^1];
        }
    }
}