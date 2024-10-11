// using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int GetImportance(IList<Employee> employees, int id)
    {
        var a = employees.Where(e => e.id == id).FirstOrDefault();
        if (a is null) { return 0; }
        else
        {
            return a.importance + a.subordinates.Select(i => GetImportance(employees, i)).Sum();
        }
    }
}

public record class Employee
{
    public int id;
    public int importance;
    public IList<int> subordinates;
}