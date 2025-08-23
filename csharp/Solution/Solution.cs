using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class LFUCache
{
    public LFUCache(int capacity)
    {
        MinF = 0;
        Cap = capacity;
        KVF = [];
        FK = [];
    }

    int MinF { get; set; }
    int Cap { get; }
    // key - (val, freq)
    Dictionary<int, (int val, int freq)> KVF { get; }
    // freq - [keys..]
    Dictionary<int, List<int>> FK { get; }

    public int Get(int key)
    {
        if (!KVF.TryGetValue(key, out var item)) { return -1; }
        FK[item.freq].Remove(key);
        if (FK[item.freq].Count == 0 && item.freq == MinF) { MinF += 1; }
        Insert(key, item.val, 1 + item.freq);
        return item.val;
    }

    public void Put(int key, int value)
    {
        if (Cap <= 0) { return; }
        if (KVF.TryGetValue(key, out var item))
        {
            KVF[key] = (value, item.freq);
            Get(key);
        }
        else
        {
            if (KVF.Count == Cap)
            {
                int del = FK[MinF][0];
                FK[MinF].RemoveAt(0);
                KVF.Remove(del);
            }
            MinF = 1;
            Insert(key, value, 1);
        }
    }

    void Insert(int key, int val, int freq)
    {
        KVF[key] = (val, freq);
        FK.TryAdd(freq, []);
        FK[freq].Add(key);
    }
}