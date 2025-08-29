using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public long FlowerGame(int n, int m)
    {
        long nn = n;
        long mm = m;
        return (nn + 1) / 2 * (mm / 2) + nn / 2 * ((mm + 1) / 2);
    }
}