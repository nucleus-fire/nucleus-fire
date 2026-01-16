<n:view title="Action Test">
<n:loader>println!("LOADER");</n:loader>
<n:action>
    println!("ACTION");
    println!("Params: {:?}", params);
    return axum::response::Redirect::to("/action_success").into_response();
</n:action>
    <div style="padding: 2rem;">
        <h1>Action Test</h1>
        <form method="POST" action="/action_test">
            <input type="text" name="username" placeholder="Enter username" />
            <button type="submit">Submit</button>
        </form>
    </div>
</n:view>
