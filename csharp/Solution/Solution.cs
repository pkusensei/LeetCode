using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class RandomizedCollection
{
    public RandomizedCollection()
    {
        Rng = new();
        ValPos = [];
        Vals = [];
    }

    Random Rng { get; }
    Dictionary<int, HashSet<int>> ValPos { get; }
    List<int> Vals { get; }
    int Count { get; set; }

    public bool Insert(int val)
    {
        bool res = ValPos.TryAdd(val, []) || ValPos[val].Count == 0;
        ValPos[val].Add(Count);
        if (Count < Vals.Count) { Vals[Count] = val; }
        else { Vals.Add(val); }
        Count += 1;
        return res;
    }

    public bool Remove(int val)
    {
        bool res = ValPos.TryGetValue(val, out var set) && set.Count > 0;
        if (!res) { return res; }
        int idx = set.First();
        set.Remove(idx);
        int swap = Vals[Count - 1];
        (Vals[idx], Vals[Count - 1]) = (swap, Vals[idx]); // swap remove
        Count -= 1;
        if (idx < Count)
        {
            ValPos[swap].Remove(Count);
            ValPos[swap].Add(idx);
        }
        return true;
    }

    public int GetRandom()
    {
        int idx = Rng.Next(Count);
        return Vals[idx];
    }
}