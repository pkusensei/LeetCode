using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public double MyPow(double x, int n)
    {
        return Dfs(x, n);

        static double Dfs(double x, long n)
        {
            if (n == 0 || x == 1.0) { return 1.0; }
            if (x == -1.0) { return (n & 1) == 0 ? 1.0 : -1.0; }
            if (x == 0.0) { return 0.0; }
            if (n < 0) { return Dfs(1.0 / x, -n); }
            if ((n & 1) == 0) { return Dfs(x * x, n >> 1); }
            return x * Dfs(x * x, n >> 1);
        }
    }
}
