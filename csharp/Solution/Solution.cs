using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class TaskManager
{
    public TaskManager(IList<IList<int>> tasks)
    {
        TaskUP = [];
        MaxHeap = new(Comparer<(int prio, int task)>.Create(
            (a, b) => a.prio == b.prio ? b.task.CompareTo(a.task) : b.prio.CompareTo(a.prio)));
        foreach (var row in tasks)
        {
            Add(row[0], row[1], row[2]);
        }
    }

    Dictionary<int, (int user, int prio)> TaskUP { get; }
    PriorityQueue<(int prio, int task), (int prio, int task)> MaxHeap { get; }

    public void Add(int userId, int taskId, int priority)
    {
        TaskUP[taskId] = (userId, priority);
        MaxHeap.Enqueue((priority, taskId), (priority, taskId));
    }

    public void Edit(int taskId, int newPriority)
    {
        int user = TaskUP[taskId].user;
        TaskUP[taskId] = (user, newPriority);
        MaxHeap.Enqueue((newPriority, taskId), (newPriority, taskId));
    }

    public void Rmv(int taskId) => TaskUP.Remove(taskId);

    public int ExecTop()
    {
        while (MaxHeap.TryDequeue(out var top, out _))
        {
            (int prio, int task) = top;
            if (TaskUP.TryGetValue(task, out var v) && v.prio == prio)
            {
                TaskUP.Remove(task);
                return v.user;
            }
        }
        return -1;
    }
}