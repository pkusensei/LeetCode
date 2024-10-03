// using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    Stack<string> Stack { get; } = [];
    bool HasTag { get; set; } = false;
    readonly string pattern = @"<([A-Z]{1,9})>([^<]*((<\/?[A-Z]{1,9}>)|(<!\[CDATA\[(.*?)]]>))?[^<]*)*<\/\1>";

    public bool IsValid(string code)
    {
        if (!System.Text.RegularExpressions.Regex.IsMatch(code, pattern))
        {
            return false;
        }
        for (int i = 0; i < code.Length; i++)
        {
            var is_end = false;
            if (Stack.Count == 0 && HasTag)
            {
                return false;
            }
            if (code[i] == '<')
            {
                if (code[i + 1] == '!')
                {
                    i = code.IndexOf("]]>", i + 1);
                    continue;
                }
                if (code[i + 1] == '/')
                {
                    i += 1;
                    is_end = true;
                }
                var close = code.IndexOf('>', i + 1);
                if (close < 0 || !IsValidTagName(code.Substring(i + 1, close - i - 1), is_end))
                {
                    return false;
                }
                i = close;
            }
        }
        return Stack.Count == 0;
    }

    bool IsValidTagName(string s, bool is_end)
    {
        if (is_end)
        {
            if (Stack.Count > 0 && Stack.Peek() == s) { Stack.Pop(); }
            else { return false; }
        }
        else
        {
            HasTag = true;
            Stack.Push(s);
        }
        return true;
    }
}
