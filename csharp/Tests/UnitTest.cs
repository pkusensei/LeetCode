using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    Solution.Solution sol = new();

    [TestMethod]
    public void TestMethod1()
    {
        var n1 = new Solution.Node() { val = 1 };
        var n2 = new Solution.Node() { val = 2 };
        var n3 = new Solution.Node() { val = 3 };
        n1.next = n2; n2.prev = n1;
        n1.child = n3;
        sol.Flatten(n1);

    }

    [TestMethod]
    public void TestMethod2()
    {
        // var b = sol.Construct([[1, 1, 1, 1, 0, 0, 0, 0], [1, 1, 1, 1, 0, 0, 0, 0], [1, 1, 1, 1, 1, 1, 1, 1], [1, 1, 1, 1, 1, 1, 1, 1], [1, 1, 1, 1, 0, 0, 0, 0], [1, 1, 1, 1, 0, 0, 0, 0], [1, 1, 1, 1, 0, 0, 0, 0], [1, 1, 1, 1, 0, 0, 0, 0]]);
        // var c = 127;
        // Assert.AreEqual(c, b);
    }
}