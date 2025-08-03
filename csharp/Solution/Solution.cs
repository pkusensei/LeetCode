using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class RandomizedSet
{
    public RandomizedSet()
    {
        Rng = new();
        ValPos = [];
        Vals = [];
    }

    Random Rng { get; }
    Dictionary<int, int> ValPos { get; }
    List<int> Vals { get; }
    int Count { get; set; }

    public bool Insert(int val)
    {
        if (ValPos.ContainsKey(val)) { return false; }
        if (Count < Vals.Count) { Vals[Count] = val; }
        else { Vals.Add(val); }
        ValPos.Add(val, Count);
        Count += 1;
        return true;
    }

    public bool Remove(int val)
    {
        if (!ValPos.TryGetValue(val, out int idx)) { return false; }
        ValPos.Remove(val);
        (Vals[idx], Vals[Count - 1]) = (Vals[Count - 1], Vals[idx]); // swap remove
        Count -= 1;
        if (idx < Count) { ValPos[Vals[idx]] = idx; }
        return true;
    }

    public int GetRandom()
    {
        int idx = Rng.Next(Count);
        return Vals[idx];
    }
}