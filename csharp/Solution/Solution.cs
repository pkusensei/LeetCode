using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<int> LexicalOrder(int n)
    {
        List<int> res = [];
        for (int i = 1; i < 10; i++)
        {
            Dfs(i);
        }
        return res;

        void Dfs(int curr)
        {
            if (curr > n) { return; }
            res.Add(curr);
            for (int i = 0; i < 10; i++)
            {
                Dfs(10 * curr + i);
            }
        }
    }
}