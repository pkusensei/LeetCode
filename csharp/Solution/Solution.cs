using System.Collections.Frozen;
using System.Linq.Expressions;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class H2O
{
    readonly object _lock;
    int hyd;
    int oxy;

    public H2O()
    {
        _lock = new();
        hyd = 0;
        oxy = 0;
    }

    public void Hydrogen(Action releaseHydrogen)
    {
        lock (_lock)
        {
            while (hyd == 2)
            {
                Monitor.Wait(_lock);
            }
            // releaseHydrogen() outputs "H". Do not change or remove this line.
            releaseHydrogen();
            hyd += 1;
            if (oxy == 1 && hyd == 2)
            {
                oxy = 0;
                hyd = 0;
            }
            Monitor.PulseAll(_lock);
        }
    }

    public void Oxygen(Action releaseOxygen)
    {
        lock (_lock)
        {
            while (oxy == 1)
            {
                Monitor.Wait(_lock);
            }
            // releaseOxygen() outputs "O". Do not change or remove this line.
            releaseOxygen();
            oxy += 1;
            if (oxy == 1 && hyd == 2)
            {
                oxy = 0;
                hyd = 0;
            }
            Monitor.PulseAll(_lock);
        }
    }
}
