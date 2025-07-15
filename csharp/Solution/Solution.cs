using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public bool IsValid(string word)
    {
        if (word.Length < 3) { return false; }
        bool vow = false;
        bool con = false;
        foreach (var c in word)
        {
            if (char.IsAsciiDigit(c)) { continue; }
            if (char.IsAsciiLetter(c))
            {
                if ("aeiou".Contains(char.ToLower(c))) { vow = true; }
                else { con = true; }
            }
            else { return false; }
        }
        return vow && con;
    }
}