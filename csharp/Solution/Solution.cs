using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public double SoupServings(int n)
    {
        if (n >= 4800) { return 1.0; }
        Dictionary<(int, int), double> memo = [];
        return Dfs(n, n);

        double Dfs(int a, int b)
        {
            if (a <= 0 && b > 0) { return 1.0; }
            if (a <= 0 && b <= 0) { return 0.5; }
            if (a > 0 && b <= 0) { return 0; }
            if (memo.TryGetValue((a, b), out var v)) { return v; }
            double res = Dfs(a - 100, b) + Dfs(a - 75, b - 25)
                         + Dfs(a - 50, b - 50) + Dfs(a - 25, b - 75);
            res /= 4;
            memo.Add((a, b), res);
            return res;
        }
    }

    public double BottomUp(int n)
    {
        if (n > 4800) { return 1; }
        int n_ = n / 25 + int.Min(1, n % 25);
        double[,] dp = new double[1 + n_, 1 + n_];
        dp[0, 0] = 1.0;
        for (int i1 = 0; i1 < n_; i1++)
        {
            for (int i2 = 0; i2 < n_; i2++)
            {
                double curr = dp[0, 0] / 4;
                if (curr == 0.0) { continue; }
                dp[int.Min(i1 + 4, n_), i2] += curr;
                dp[int.Min(i1 + 3, n_), int.Min(1 + i2, n_)] += curr;
                dp[int.Min(i1 + 2, n_), int.Min(2 + i2, n_)] += curr;
                dp[int.Min(i1 + 1, n_), int.Min(3 + i2, n_)] += curr;
            }
        }
        double res = dp[n_, n_] / 2;
        for (int i = 0; i < n_; i++)
        {
            res += dp[n_, i];
        }
        return res;
    }
}
