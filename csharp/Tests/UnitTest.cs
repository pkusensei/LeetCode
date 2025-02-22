using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    public void TestMethod1()
    {
        var a = sol.RecoverFromPreorder("1-2--3--4-5--6--7");
        Assert.AreEqual("[1,2,5,3,4,6,7]", a.ToString());
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = sol.RecoverFromPreorder("1-2--3---4-5--6---7");
        Assert.AreEqual("[1,2,5,3,null,6,null,4,null,7]", a.ToString());
    }

    [TestMethod]
    public void TestMethod3()
    {
        var a = sol.RecoverFromPreorder("1-401--349---90--88");
        Assert.AreEqual("[1,401,null,349,88,90]", a.ToString());
    }

    [TestMethod]
    public void TestMethod4()
    {
    }
}