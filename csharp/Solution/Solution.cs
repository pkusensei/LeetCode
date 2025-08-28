using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int LargestPalindrome(int n)
    {
        long up = (long)Math.Pow(10, n) - 1;
        long down = (long)Math.Pow(10, n - 1);
        for (long i = up; i >= down; i -= 1)
        {
            string s = i.ToString();
            StringBuilder sb = new(s);
            sb.Append([.. s.Reverse()]);
            if (long.TryParse(sb.ToString(), out var v) && Check(v))
            {
                return (int)(v % 1337);
            }
        }
        return 9;

        bool Check(long num)
        {
            for (long i = up; i >= down; i -= 1)
            {
                if (i * i < num) { break; }
                if (num % i == 0)
                {
                    long val = num / i;
                    if (down <= val && val <= up) { return true; }
                }
            }
            return false;
        }
    }
}