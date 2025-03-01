// Validate status 4xx
pm.test("[POST]::/payments - Status code is 4xx", function () {
  pm.response.to.be.error;
});

// Validate if response header has matching content-type
pm.test(
  "[POST]::/payments/:id/confirm - Content-Type is application/json",
  function () {
    pm.expect(pm.response.headers.get("Content-Type")).to.include(
      "application/json",
    );
  },
);

// Validate if response has JSON Body
pm.test("[POST]::/payments/:id/confirm - Response has JSON Body", function () {
  pm.response.to.have.jsonBody();
});

// Set response object as internal variable
let jsonData = {};
try {
  jsonData = pm.response.json();
} catch (e) {}

// pm.collectionVariables - Set payment_id as variable for jsonData.payment_id
if (jsonData?.payment_id) {
  pm.collectionVariables.set("payment_id", jsonData.payment_id);
  console.log(
    "- use {{payment_id}} as collection variable for value",
    jsonData.payment_id,
  );
} else {
  console.log(
    "INFO - Unable to assign variable {{payment_id}}, as jsonData.payment_id is undefined.",
  );
}

// pm.collectionVariables - Set mandate_id as variable for jsonData.mandate_id
if (jsonData?.mandate_id) {
  pm.collectionVariables.set("mandate_id", jsonData.mandate_id);
  console.log(
    "- use {{mandate_id}} as collection variable for value",
    jsonData.mandate_id,
  );
} else {
  console.log(
    "INFO - Unable to assign variable {{mandate_id}}, as jsonData.mandate_id is undefined.",
  );
}

// pm.collectionVariables - Set client_secret as variable for jsonData.client_secret
if (jsonData?.client_secret) {
  pm.collectionVariables.set("client_secret", jsonData.client_secret);
  console.log(
    "- use {{client_secret}} as collection variable for value",
    jsonData.client_secret,
  );
} else {
  console.log(
    "INFO - Unable to assign variable {{client_secret}}, as jsonData.client_secret is undefined.",
  );
}

// Response body should have "error"
pm.test(
  "[POST]::/payments/:id/confirm - Content check if 'error' exists",
  function () {
    pm.expect(typeof jsonData.error !== "undefined").to.be.true;
  },
);

// Response body should have value "connector error" for "error type"
if (jsonData?.error?.type) {
  pm.test(
    "[POST]::/payments/:id/confirm - Content check if value for 'error.type' matches 'invalid_request'",
    function () {
      pm.expect(jsonData.error.type).to.eql("invalid_request");
    },
  );
}

// Response body should have value "A payment token or payment method data or ctp service details is required" for "message"
if (jsonData?.error?.message) {
  pm.test(
    "[POST]::/payments - Content check if value for 'error.message' matches 'connector_error'",
    function () {
      pm.expect(jsonData.error.message).to.eql(
        "A payment token or payment method data or ctp service details is required",
      );
    },
  );
}
