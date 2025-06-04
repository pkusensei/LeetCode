using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public string AnswerString(string word, int numFriends)
    {
        if (numFriends <= 1) { return word; }
        int len = word.Length - numFriends + 1;
        string res = "";
        for (int i = 0; i < word.Length; i++)
        {
            string curr = word[i..Math.Min(i + len, word.Length)];
            res = curr.CompareTo(res) > 0 ? curr : res;
        }
        return res;
    }
}
