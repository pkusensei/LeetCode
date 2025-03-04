using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public bool CheckPowersOfThree(int n)
    {
        // Each power of 3 can have at most one copy
        // That is, after converting number to base 3
        // Each digit is at most 1
        while (n % 3 != 2 && n > 0) { n /= 3; }
        return n == 0;
    }
}
