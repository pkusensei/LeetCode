using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int CanBeTypedWords(string text, string brokenLetters)
    {
        int res = 0;
        foreach (var item in text.Split(' '))
        {
            if (!item.Intersect(brokenLetters).Any()) { res += 1; }
        }
        return res;
    }
}